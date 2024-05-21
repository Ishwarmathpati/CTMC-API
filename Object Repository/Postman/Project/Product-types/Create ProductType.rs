<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create ProductType</name>
   <tag></tag>
   <elementGuidId>74c50ded-5cc7-47cb-b4f9-0dcaa270523d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;name\&quot; : \&quot;test_product_type\&quot;,\n  \&quot;description\&quot; : \&quot;Test product type.\&quot;,\n  \&quot;attributes\&quot; : [ {\n    \&quot;type\&quot; : {\n      \&quot;name\&quot; : \&quot;text\&quot;\n    },\n    \&quot;isSearchable\&quot; : false,\n    \&quot;inputHint\&quot; : \&quot;SingleLine\&quot;,\n    \&quot;name\&quot; : \&quot;size\&quot;,\n    \&quot;label\&quot; : {\n      \&quot;en\&quot; : \&quot;The right size is important.\&quot;\n    },\n    \&quot;isRequired\&quot; : false,\n    \&quot;attributeConstraint\&quot; : \&quot;CombinationUnique\&quot;\n  } ]\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer {{ctp_access_token}}</value>
      <webElementGuid>48e1d726-fb2b-459e-846c-454460059504</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>1c33b4ba-a66d-412c-8a86-254936e8e8e9</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/product-types</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.host</defaultValue>
      <description></description>
      <id>8276c155-a021-4a51-baea-5ed841cdb133</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>88d36f50-8358-4bba-97f8-78db6a30fd56</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
