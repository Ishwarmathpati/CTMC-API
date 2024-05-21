<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SetCategoryOrderHint</name>
   <tag></tag>
   <elementGuidId>b2e96c29-0cc4-473c-b0f2-5142cebc5978</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${product-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;setCategoryOrderHint\&quot;,\n            \&quot;categoryId\&quot; : \&quot;${category-id}\&quot;,\n            \&quot;orderHint\&quot; : \&quot;0.1\&quot;\n          }\n    ]\n}&quot;,
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
      <webElementGuid>128508f7-e4e4-435d-b30c-fd69639bcc43</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>0a4073e3-7a37-440a-949e-c2b3142705eb</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/products/${product-id}</restUrl>
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
      <id>9a947fad-ddc9-461a-8274-4fadd3e6e318</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>05fe2f9f-53e8-442c-82e1-65f81fe4da5d</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.product-id</defaultValue>
      <description></description>
      <id>755aaa81-44a1-45bf-b5c7-1c383b6193d3</id>
      <masked>false</masked>
      <name>product-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.product-version</defaultValue>
      <description></description>
      <id>c9e8e7aa-1639-4220-913f-50fe850e4929</id>
      <masked>false</masked>
      <name>product-version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.category-id</defaultValue>
      <description></description>
      <id>1aa8ba09-38df-49f3-b54c-231500767f79</id>
      <masked>false</masked>
      <name>category-id</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
