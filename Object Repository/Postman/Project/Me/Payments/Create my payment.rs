<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create my payment</name>
   <tag></tag>
   <elementGuidId>6969437f-d56e-4fbd-8975-af06ed4f17f7</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;amountPlanned\&quot; : {\n    \&quot;currencyCode\&quot; : \&quot;USD\&quot;,\n    \&quot;centAmount\&quot; : 1000\n  },\n  \&quot;paymentMethodInfo\&quot; : {\n    \&quot;paymentInterface\&quot; : \&quot;STRIPE\&quot;,\n    \&quot;method\&quot; : \&quot;CREDIT_CARD\&quot;,\n    \&quot;name\&quot; : {\n      \&quot;en\&quot; : \&quot;Credit Card\&quot;\n    }\n  },\n  \&quot;transaction\&quot; : {\n    \&quot;timestamp\&quot; : \&quot;2015-10-20T08:54:24.000Z\&quot;,\n    \&quot;type\&quot; : \&quot;Charge\&quot;,\n    \&quot;amount\&quot; : {\n      \&quot;currencyCode\&quot; : \&quot;USD\&quot;,\n      \&quot;centAmount\&quot; : 1000\n    }\n  }\n}&quot;,
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
      <webElementGuid>55edf9fd-8504-476f-ade5-064b6bb6ffb8</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>9e6884fe-936f-4153-b9ab-eebb72a7351d</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/me/payments</restUrl>
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
      <id>9c12262f-79ea-4276-a2fa-14f0f71dbf80</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>db107f75-69b4-4b6a-8110-2bb52866dfaa</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
